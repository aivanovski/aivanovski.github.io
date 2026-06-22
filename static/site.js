const root = document.documentElement;

function addLineNumbers(codeBlock) {
  const source = codeBlock.cloneNode(true);
  const lines = [document.createElement("span")];

  function appendText(text, ancestors) {
    text.split("\n").forEach((part, index) => {
      if (index > 0) {
        lines.push(document.createElement("span"));
      }

      let target = lines[lines.length - 1];

      ancestors.forEach((ancestor) => {
        const clone = ancestor.cloneNode(false);
        target.append(clone);
        target = clone;
      });

      target.append(document.createTextNode(part));
    });
  }

  function visit(node, ancestors = []) {
    if (node.nodeType === Node.TEXT_NODE) {
      appendText(node.textContent ?? "", ancestors);
      return;
    }

    if (node.nodeType === Node.ELEMENT_NODE) {
      node.childNodes.forEach((child) => visit(child, [...ancestors, node]));
    }
  }

  source.childNodes.forEach((node) => visit(node));

  if (lines.length > 1 && lines[lines.length - 1].textContent === "") {
    lines.pop();
  }

  lines.forEach((line) => line.classList.add("code-line"));
  codeBlock.replaceChildren(...lines);
}

document.querySelectorAll(".article-prose pre code").forEach((codeBlock) => {
  codeBlock.parentElement?.classList.add("code-block");

  if (window.hljs) {
    window.hljs.highlightElement(codeBlock);
  }

  addLineNumbers(codeBlock);
});

function applyTheme(theme) {
  root.classList.remove("light", "dark");
  root.classList.add(theme);
  document.getElementById("highlight-theme-dark").media = theme === "dark" ? "all" : "not all";
  document.getElementById("highlight-theme-light").media = theme === "light" ? "all" : "not all";
  localStorage.setItem("theme", theme);
}

document.querySelectorAll("[data-theme-toggle]").forEach((button) => {
  button.addEventListener("click", () => {
    const nextTheme = root.classList.contains("dark") ? "light" : "dark";
    applyTheme(nextTheme);
  });
});

const drawer = document.querySelector("[data-menu]");
const menuToggles = document.querySelectorAll("[data-menu-toggle]");
const menuClosers = document.querySelectorAll("[data-menu-close]");
const drawerLinks = drawer ? drawer.querySelectorAll("a") : [];

function closeDrawer() {
  if (!drawer) {
    return;
  }

  drawer.hidden = true;
  document.body.classList.remove("drawer-open");
}

function openDrawer() {
  if (!drawer) {
    return;
  }

  drawer.hidden = false;
  document.body.classList.add("drawer-open");
}

menuToggles.forEach((toggle) => {
  toggle.addEventListener("click", openDrawer);
});

menuClosers.forEach((closer) => {
  closer.addEventListener("click", closeDrawer);
});

drawerLinks.forEach((link) => {
  link.addEventListener("click", closeDrawer);
});

document.addEventListener("keydown", (event) => {
  if (event.key === "Escape") {
    closeDrawer();
  }
});
