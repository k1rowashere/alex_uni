use leptos::*;
use std::collections::{BTreeSet, HashMap};

use super::server_fns::{get_registerable_subjects, get_subbed_subjects};
use super::{Subject, SubjectChoices, SubjectId};

use crate::class::Class;
use crate::components::suserr::TransErrs;

#[derive(Debug)]
struct MapValue {
    classes: Vec<Class>,
    is_selected: bool,
    initial_selected: bool,
    subject_idx: usize,
}

#[derive(Copy, Clone)]
pub struct SubjectsSignal {
    subject_map: RwSignal<HashMap<SubjectId, MapValue>>,
    collision_map: RwSignal<[[Vec<SubjectId>; 12]; 6]>,
    subjects_choices: StoredValue<Vec<SubjectChoices>>,
}

#[component]
pub fn CxtProvider<F>(children: F) -> impl IntoView
where
    F: Fn(SubjectsSignal) -> Fragment + 'static,
{
    let all = Resource::new(|| (), |_| get_registerable_subjects());
    let selected = Resource::new(|| (), |_| get_subbed_subjects());

    view! {
        <TransErrs r1=all r2=selected let:all let:selected>
        {
            let signal = SubjectsSignal::new(selected, all);
            provide_context(signal);
            children(signal)
        }
        </TransErrs>
    }
}

impl SubjectsSignal {
    pub fn new(selected: &BTreeSet<SubjectId>, all: &[SubjectChoices]) -> Self {
        let subjects: HashMap<_, _> = all
            .iter()
            .enumerate()
            .flat_map(|(i, s)| s.choices.iter().zip(std::iter::repeat(i)))
            .map(|(subject, i)| {
                let Subject { id, lec, tut, lab, .. } = subject.clone();
                let classes =
                    [Some(lec), tut, lab].into_iter().flatten().collect();
                let is_selected = selected.contains(&id);
                let initial_selected = is_selected;
                (
                    id,
                    MapValue {
                        classes,
                        is_selected,
                        initial_selected,
                        subject_idx: i,
                    },
                )
            })
            .collect();

        // init collision map
        let collision_map = {
            let mut map: [[Vec<SubjectId>; 12]; 6] = Default::default();
            for subject in selected {
                let classes = &subjects.get(subject).unwrap().classes;
                for class in classes {
                    let Class { day, period: (st, end), .. } = class;
                    for i in *st..*end {
                        map[*day as usize][i].push(*subject);
                    }
                }
            }
            RwSignal::new(map)
        };

        Self {
            subject_map: RwSignal::new(subjects),
            collision_map,
            subjects_choices: StoredValue::new(all.to_vec()),
        }
    }

    pub fn save(self) {
        use super::server_fns::register_subjects;
        spawn_local(async move {
            let selected = self.subject_map.with_untracked(|hm| {
                hm.iter()
                    .filter(|(_, v)| v.is_selected)
                    .map(|(k, _)| *k)
                    .collect()
            });
            // TODO: handle errors (show error msg)
            let _ = register_subjects(selected).await;
            // if success
            self.subject_map.update(|hm| {
                hm.values_mut()
                    .for_each(|v| v.initial_selected = v.is_selected);
            });
        })
    }

    pub fn saved(self) -> Signal<bool> {
        Memo::new(move |_| {
            self.subject_map.with(|hm| {
                hm.values().all(|v| v.is_selected == v.initial_selected)
            })
        })
        .into()
    }

    pub fn discard(self) {
        self.subject_map.update(|m| {
            m.values_mut()
                .for_each(|v| v.is_selected = v.initial_selected);
        });
    }

    /// returns a signal that emits true if the subject is selected
    pub fn is_selected(self, subject: SubjectId) -> Signal<bool> {
        Memo::new(move |_| {
            self.subject_map.with(move |hm| {
                matches!(
                    hm.get(&subject),
                    Some(MapValue { is_selected: true, .. })
                )
            })
        })
        .into()
    }

    /// returns a signal that emits true if the subject has changed from initial state
    pub fn has_changed(self, subject: SubjectId) -> Signal<bool> {
        Memo::new(move |_| {
            self.subject_map.with(move |hm| {
                matches!(
                    hm.get(&subject),
                    Some(MapValue { is_selected, initial_selected, .. })
                    if *is_selected != *initial_selected
                )
            })
        })
        .into()
    }

    pub fn is_selected_untracked(self, subject: SubjectId) -> bool {
        self.subject_map.with_untracked(move |hm| {
            matches!(hm.get(&subject), Some(MapValue { is_selected: true, .. }))
        })
    }

    pub fn select(self, subject: SubjectId) {
        let subjects = self.subject_map;
        let col_map = self.collision_map;
        batch(|| {
            update!(|subjects, col_map| {
                let (classes, idx) = match subjects.get_mut(&subject) {
                    Some(MapValue { is_selected: true, .. }) | None => return,
                    Some(MapValue {
                        classes,
                        is_selected,
                        subject_idx,
                        ..
                    }) => {
                        *is_selected = true;
                        (classes, *subject_idx)
                    }
                };

                for class in classes {
                    let Class { day, period: (st, end), .. } = *class;
                    for i in st..end {
                        col_map[day as usize][i].push(subject);
                    }
                }

                // unselect all subjects with the same idx from subject choices
                self.subjects_choices.with_value(|choices| {
                    choices[idx]
                        .choices
                        .iter()
                        .map(|s| s.id)
                        .filter(|&id| subject != id)
                        .for_each(|id| {
                            Self::deselect_helper(id, subjects, col_map)
                        })
                });
            });
        });
    }

    fn deselect_helper(
        id: SubjectId,
        subjects: &mut HashMap<SubjectId, MapValue>,
        col_map: &mut [[Vec<SubjectId>; 12]; 6],
    ) {
        let classes = match subjects.get_mut(&id) {
            Some(MapValue { is_selected: false, .. }) | None => return,
            Some(MapValue { classes, is_selected, .. }) => {
                *is_selected = false;
                classes
            }
        };

        for class in classes {
            let Class { day, period: (st, end), .. } = *class;
            for i in st..end {
                col_map[day as usize][i].retain(|&el| el != id);
            }
        }
    }

    pub fn deselect(self, subject: SubjectId) {
        let subjects = self.subject_map;
        let col_map = self.collision_map;
        update!(|subjects, col_map| {
            Self::deselect_helper(subject, subjects, col_map)
        });
    }

    pub fn toggle(self, subject: SubjectId) {
        if self.is_selected_untracked(subject) {
            self.deselect(subject);
        } else {
            self.select(subject);
        }
    }

    pub fn has_collisions(self, subject: SubjectId) -> Signal<bool> {
        let class_idx = self.subject_map.with_untracked(|hm| {
            let classes = hm.get(&subject).map(|v| &v.classes)?;
            let class_idx: Vec<_> = classes
                .iter()
                .flat_map(|c| {
                    (c.period.0..c.period.1).map(|i| (c.day as usize, i))
                })
                .collect();
            Some(class_idx)
        });

        match class_idx {
            Some(class_idx) => Memo::new(move |_| {
                self.collision_map.with(|col_map| {
                    class_idx
                        .iter()
                        .map(|&(day, period)| &col_map[day][period])
                        .any(|v| v.contains(&subject) && v.len() > 1)
                })
            })
            .into(),
            None => Signal::from(|| false),
        }
    }

    pub fn classes(self) -> Signal<Vec<Class>> {
        // PERF: this could be optimized with memos and stuff
        (move || {
            self.subject_map.with(|hm| {
                hm.values()
                    .filter(|v| v.is_selected)
                    .flat_map(|v| v.classes.clone())
                    .collect()
            })
        })
        .into_signal()
    }

    pub fn choices(self) -> StoredValue<Vec<SubjectChoices>> {
        self.subjects_choices
    }
}
