My personal website built in vanilla JS/HTML/CSS + Rust WASM for some spice.

Using minimal libraries as a learning exercise.

Crates like gloo/futures were used to better my understanding of async Rust.

## roadmap

Towards building a fully functional single page application using client-side hash routing.

### pages

#### Home - a summary of all the other pages

Ideally nobody needs to go to the other pages

- [ ] add back image dots to hero
- [ ] incorporate other pages when done

#### Portfolio - all photography/videography work I want to show off

For small things, static serving is fine, but as I grow my portfolio this doesn't scale.

- [ ] design skeleton
- [ ] host all pictures on Google Drive and videos on Youtube
- [ ] dynamically fetch images + videos onto page

Will want to profile this for performance later on, but some things to consider:

- [ ] webp compression pipeline (R.I.P. JPEG-XL)

#### Run log - strava api, running blogs, current training

Will document my running journey, goals, current training data on this page.

May eventually expand this page to fitness in general and not just running.

- [ ] design a skeleton
- [ ] strava API

##### run trends visualisation

It would be cool if I could use the strava data to show some trends.

- [ ] HR over time
- [ ] frontend clustering algorithm for type of run -> how much strava data can I fetch?
- [ ] speed over time


#### Blogs - my writings, thoughts, feelings

Show my thoughts on a variety of topics: reviews, creative writing, thoughts on xyz, etc.

- [ ] design skeleton
- [ ] design obsidian note <-> website link through github and fetching
- [ ] md parsing and html generation

### misc

- [ ] favico?
- [ ] head titles
- [ ] making the home page static as well so you can disable js and it'll work 50%.
- [ ] mobile friendliness
