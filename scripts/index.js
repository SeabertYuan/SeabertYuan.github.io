const burger = document.querySelector(".burger");
const navLinks = document.querySelector(".navLinks");
const links = document.querySelectorAll(".navLinks a");

burger.addEventListener("click", () => {
    navLinks.classList.toggle("open");

    burger.classList.toggle("toggle");
});

const quotes = [
    "don't stop dreaming",
    "live for yourself",
    "create against all odds",
    "you won't know what you have until you lose it",
    "home-made"
];

function getRandomInt(max) {
    return Math.floor(Math.random() * max);
}

const elmId = "mainSubtitle";
const value = quotes[getRandomInt(5)];

function updateHTML(elmId, value) {
  var elem = document.getElementById(elmId);
  if(typeof elem !== 'undefined' && elem !== null) {
    elem.textContent = value;
    console.log("yes");
  }
}

$(document).ready(() => {
    updateHTML(elmId, value);
  }
);
