let scrollTimeout;

const listenActive = () => {
  const elems = document.querySelector(".pagetoc").children;
  [...elems].forEach(el => {
    el.addEventListener("click", (event) => {
      clearTimeout(scrollTimeout);
      [...elems].forEach(el => el.classList.remove("active"));
      el.classList.add("active");
      // Prevent scroll updates for a short period
      scrollTimeout = setTimeout(() => {
        scrollTimeout = null;
      }, 100); // Adjust timing as needed
    });
  });
};

const getPagetoc = () => document.querySelector(".pagetoc") || autoCreatePagetoc();

const autoCreatePagetoc = () => {
  const main = document.querySelector("#content > main");
  const content = Object.assign(document.createElement("div"), {
    className: "content-wrap"
  });
  content.append(...main.childNodes);
  main.prepend(content);
  main.insertAdjacentHTML("afterbegin", '<div class="sidetoc"><nav class="pagetoc"></nav></div>');
  return document.querySelector(".pagetoc");
};


const updateOnScroll = () => {
  if (scrollTimeout) return;
  const headers = [...document.getElementsByClassName("header")];
  const lastHeader = headers.reverse().find(el => window.scrollY >= el.offsetTop);
  [...document.querySelector(".pagetoc").children].forEach(link => {
    link.classList.remove("active");
    if (lastHeader && lastHeader.href === link.href) {
      link.classList.add("active");
    }
  });
};

window.addEventListener('load', () => {
  const pagetoc = getPagetoc();
  const headers = [...document.getElementsByClassName("header")];
  headers.forEach(header => {
    const link = Object.assign(document.createElement("a"), {
      textContent: header.text,
      href: header.href,
      className: `pagetoc-${header.parentElement.tagName}`
    });
    pagetoc.appendChild(link);
  });
  updateOnScroll();
  listenActive();
  window.addEventListener("scroll", updateOnScroll);
});

