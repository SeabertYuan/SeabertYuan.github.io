const burger = document.querySelector(".burger");
const navLinks = document.querySelector(".navLinks");
const links = document.querySelectorAll(".navLinks a");

burger.addEventListener("click", () => {
    navLinks.classList.toggle("open");

    burger.classList.toggle("toggle");
});