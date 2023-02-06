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

window.onresize = function(event) {
    document.getElementById('heroTitle').style.height = (window.innerHeight - 180)+'px'
    document.getElementById('mainSection').style.height = (window.innerHeight)+'px';
}

$(document).ready(() => {
    updateHTML(elmId, value);
    //$('.mainSection').style.height = (document.height)+'px';
    document.getElementById('mainSection').style.height = (window.innerHeight)+'px';
    $('.hero-slideshow').slick({
      autoplay: true,
      autoplaySpeed: 4000,
      speed: 1000,
      fade: true,
      dots: true,
      arrows: false,
      fade: true,
      dotsClass: "slide-dots",
      pauseOnHover: false,
      pauseOnDotsHover: false,
      waitForAnimate: false,
      pauseOnFocus: false,
    });
  }
);
