const systemTheme = window.matchMedia("(prefers-color-scheme: dark)").matches;
const theme = localStorage.getItem("theme");
if (
    theme === "dark" ||
    ((theme === null || theme === "system") && systemTheme)
) {
    document.documentElement.classList.add("dark");
}
