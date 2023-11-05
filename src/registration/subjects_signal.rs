use leptos::*;
use std::collections::{BTreeSet, HashMap};

use super::server_fns::{get_registerable_subjects, get_subbed_subjects};
use super::{Subject, SubjectChoices, SubjectId};

use crate::class::Class;
use crate::components::suserr::TransErrs;

#[derive(Copy, Clone)]
pub struct SubjectsSignal {
    subjects: RwSignal<HashMap<SubjectId, (Vec<Class>, bool)>>,
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
        let subjects = all
            .iter()
            .cloned()
            .flat_map(|s| s.choices)
            .map(|s| {
                let Subject { id, lec, tut, lab, .. } = s;
                let classes =
                    [Some(lec), tut, lab].into_iter().flatten().collect();
                let is_selected = selected.contains(&id);
                (id, (classes, is_selected))
            })
            .collect();

        // TODO: init collision map

        Self {
            subjects: RwSignal::new(subjects),
            collision_map: Default::default(),
            subjects_choices: StoredValue::new(all.to_vec()),
        }
    }

    pub fn save(self) {
        todo!()
    }

    pub fn discard(self) {
        todo!()
    }

    // TODO:
    pub fn loading(self) -> Signal<bool> {
        Signal::from(|| false)
    }

    /// returns a signal that emits true if the subject is selected
    pub fn is_selected(self, subject: SubjectId) -> Memo<bool> {
        Memo::new(move |_| {
            self.subjects
                .with(move |hm| matches!(hm.get(&subject), Some((_, true))))
        })
    }

    pub fn is_selected_untracked(self, subject: SubjectId) -> bool {
        self.subjects.with_untracked(move |hm| {
            matches!(hm.get(&subject), Some((_, true)))
        })
    }

    pub fn select(self, subject: SubjectId) {
        let subjects = self.subjects;
        let col_map = self.collision_map;
        batch(|| {
            update!(|subjects, col_map| {
                let classes = match subjects.get_mut(&subject) {
                    Some((_, true)) | None => return,
                    Some((classes, selected)) => {
                        *selected = true;
                        classes
                    }
                };

                for class in classes {
                    let Class { day, period: (st, end), .. } = *class;
                    for i in st..end {
                        col_map[day as usize][i].push(subject);
                    }
                }
            });
        });
    }

    pub fn deselect(self, subject: SubjectId) {
        let subjects = self.subjects;
        let col_map = self.collision_map;
        batch(|| {
            update!(|subjects, col_map| {
                let classes = match subjects.get_mut(&subject) {
                    Some((_, false)) | None => return,
                    Some((classes, selected)) => {
                        *selected = false;
                        classes
                    }
                };

                for class in classes {
                    let Class { day, period: (st, end), .. } = *class;
                    for i in st..end {
                        col_map[day as usize][i].retain(|&el| el != subject);
                    }
                }
            });
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
        let class_idx = self.subjects.with_untracked(|hm| {
            let classes = hm.get(&subject).map(|v| &v.0)?;
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
        (move || {
            self.subjects.with(|hm| {
                hm.values()
                    .filter(|v| v.1)
                    .flat_map(|v| v.0.clone())
                    .collect()
            })
        })
        .into_signal()
    }

    pub fn choices(self) -> StoredValue<Vec<SubjectChoices>> {
        self.subjects_choices
    }
}
