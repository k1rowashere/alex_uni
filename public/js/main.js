["htmx:afterSettle", "DOMContentLoaded"].forEach((e) => {
  document.addEventListener(e, () => {
    let path = window.location.pathname;
    document.querySelectorAll("a").forEach((a) => {
      if (a.ariaCurrent === "page") {
        a.classList.remove("pointer-events-none");
        a.ariaCurrent = null;
      }
      if (a.pathname === path) {
        a.classList.add("pointer-events-none");
        a.ariaCurrent = "page";
      }
    });
  });
});
htmx.config.selfRequestsOnly = false;

document.body.addEventListener("htmx:validateUrl", function (evt) {
  // only allow requests to the current server or data:*
  if (!evt.detail.sameHost && evt.detail.url.href.indexOf("data:") !== 0) {
    evt.preventDefault();
  }
});

function theme(event) {
  const root = document.documentElement;
  const isDark =
    event.target.value === "dark" ||
    (event.target.value === "auto" &&
      window.matchMedia("(prefers-color-scheme: dark)").matches);

  root.classList.toggle("dark", isDark);
}

function on_sidebar_toggle(event, dismiss_dir) {
  init_swipe(event.target, () => event.target.hidePopover(), dismiss_dir);

  document
    .querySelectorAll("body>:not(dialog)")
    .forEach((el) => (el.inert = !el.inert));

  // BUG: https://bugzilla.mozilla.org/show_bug.cgi?id=1834877
  if (event.oldState === "open")
    delayed_class_toggle(event.target, "!block", 275);
}

function init_swipe(element, hideFn, dismiss_dir) {
  if (element.classList.contains("swipe-init")) return;
  element.classList.add("swipe-init");

  let dir = dismiss_dir === "left" || dismiss_dir === "down" ? -1 : 1;
  let xaxis = dismiss_dir === "right" || dismiss_dir === "left";

  let start = 0,
    current = 0,
    touchStartTime = 0;

  element.addEventListener("touchstart", (e) => {
    start = current = xaxis ? e.touches[0].clientX : e.touches[0].clientY;
    touchStartTime = Date.now();
  });

  element.addEventListener("touchmove", (e) => {
    current = xaxis ? e.touches[0].clientX : e.touches[0].clientY;
    let dx = Math.max(0, (current - start) * dir) * dir;
    element.style.setProperty("--backdrop-opacity", 1 - Math.abs(dx) / 400);
    element.style.transform = `translate${xaxis ? "X" : "Y"}(${dx}px)`;
    element.style.transition = "none";
  });

  element.addEventListener("touchend", () => {
    const delta = current - start;
    const dt = Date.now() - touchStartTime;
    if (delta * dir > 150 || (delta * dir > 30 && dt < 200)) hideFn();
    element.style = null;
  });
}

function animate_switch(e) {
  let indicator = e.currentTarget.querySelector(".bk");
  indicator.style.setProperty(
    "--move-x",
    `${e.target.parentElement.offsetLeft}px`,
  );
  delayed_class_toggle(indicator, "animate-[scale_0.15s_ease]");
}

function delayed_class_toggle(el, cls, delay = 150) {
  el.classList.add(cls);
  setTimeout(() => el.classList.remove(cls), delay);
}
