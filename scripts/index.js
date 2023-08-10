const burger = document.querySelector(".burger");
const navLinks = document.querySelector(".navLinks");
const links = document.querySelectorAll(".navLinks a");

burger.addEventListener("click", () => {
    navLinks.classList.toggle("open");

    burger.classList.toggle("toggle");
});

const quotes = [
    "so... the weather's nice?",
    "live life without regrets.",
    "two broken hearts won't mend each other.",
    "1 Corinthians 13:4 | Love is patient, love is kind. It does not envy, it does not boast, it is not proud.",
    "home-made",
    "the race does not end after you cross the finish line.",
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
    //console.log("yes");
  }
}

window.onresize = function(event) {
    document.getElementById('heroTitle').style.top = (window.innerHeight - 280)+'px'
    document.getElementById('mainSection').style.height = (window.innerHeight)+'px';
}

$(document).ready(() => {
    updateHTML(elmId, value);
    //$('.mainSection').style.height = (document.height)+'px';
    document.getElementById('mainSection').style.height = (window.innerHeight)+'px';
    document.getElementById('heroTitle').style.top = (window.innerHeight - 280)+'px'
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
