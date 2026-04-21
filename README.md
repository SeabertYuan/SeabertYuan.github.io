My personal website built in vanilla JS/HTML/CSS + Rust WASM for some spice.

Using minimal libraries as a learning exercise.

Crates like gloo/futures were used to better my understanding of async Rust.

# roadmap

Towards building a fully functional single page application using client-side hash routing.

## next engineering challenges

<details>

<summary>What I am immediately focusing on in the next few weeks</summary>

### run log challenges

To avoid overengineering, the goal will be to implement a smart GitHub action which will periodically refresh the token, and update a gist.

Frontend will fetch that gist file and display the data on the page.

#### phase 1

naively get the system working:

- [ ] write script to update tokens
- [ ] write script to fetch activities + diff + log new into gist file
    - will be challenging to handle activity edits
- [ ] write github action to do these things
- [ ] hook up frontend to show this gist data

#### phase 2

attempt to optimise the fetching such that the website updates in apparent close-to-real-time.

- [ ] run the chron job often during specific times
- [ ] add more guards to the script to optimise API usage
- [ ] optimise frontend polling to reduce data stale periods

### blog challenges

#### phase 1

- [ ] create an action triggered when a note is added/updated in a specific folder in my personal obsidian vault
- [ ] hook up the frontend to fetch the gist and display it

#### phase 2

- [ ] write md -> html code
- [ ] hook up github action to build html code and put it into the gist.

</details>

## pages

### Home - a summary of all the other pages

Ideally nobody needs to go to the other pages

- [x] add back image dots to hero
- [ ] incorporate other pages when done

### Portfolio - all photography/videography work I want to show off

For small things, static serving is fine, but as I grow my portfolio this doesn't scale.

- [x] design skeleton
- [ ] host all pictures on Google Drive and videos on Youtube
- [ ] dynamically fetch images + videos onto page

Will want to profile this for performance later on, but some things to consider:

- [ ] Compression pipeline (JPEG-XL is... back??)

### Run log - strava api, running blogs, current training

Will document my running journey, goals, current training data on this page.

May eventually expand this page to fitness in general and not just running.

- [x] design a skeleton
- [ ] strava API

#### run trends visualisation

It would be cool if I could use the strava data to show some trends.

- [ ] HR over time
- [ ] frontend clustering algorithm for type of run -> how much strava data can I fetch?
- [ ] speed over time


### Blogs - my writings, thoughts, feelings

Show my thoughts on a variety of topics: reviews, creative writing, thoughts on xyz, etc.

- [x] design skeleton
- [ ] design obsidian note <-> website link through github and fetching
- [ ] md parsing and html generation

### misc

- [ ] favico?
- [ ] head titles
- [ ] making the home page static as well so you can disable js and it'll work 50%.
- [ ] mobile friendliness
