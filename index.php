<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="description" content="portfolio">
        <meta name="author" content="Seabert Yuan">

        <title>Home - Seabert Yuan</title>

        <link rel="preconnect" href="https://fonts.gstatic.com">
        <link href="https://fonts.googleapis.com/css2?family=Poppins:wght@300&display=swap" rel="stylesheet">

        <link rel="stylesheet" href="styles/styles.css">
        <link rel="stylesheet" href="styles/main.css">
    </head>
    <body>
        <div class="navBarContainer">
            <nav class="navBar">
                <div class="pageLabel">
                    <label>home</label>
                </div>
                <div class="emptyNavSpace"></div>
                <div class="burger">
                    <span class="line1"></span>
                    <span class="line2"></span>
                    <span class="line3"></span>
                </div>
                <div class="navLinks">
                    <a href="#">home</a>
                    <a href="#">about</a>
                    <a href="pages/projects.html">projects</a>
                    <a href="#">contact</a>
                </div>
            </nav>
        </div>
        <script src="/scripts/index.js"></script>
        <section id="mainSection" class="pageSections">
            <div class="mainContainer">
                <div id="mainCell" class="cell">
                    <h1 class="mainTitle">seabert yuan</h1>
                    <h2 class="mainSubtitle">don't stop dreaming</h2>
                </div>
            </div>
        </section>

        <section id="aboutSection" class="pageSections">
            <div id="aboutText" class="container">
                <div id="aboutTitle" class="cell">
                    <h2>about me</h2>
                </div>
                <div id="aboutDesc" class="cell">
                    <p>student.<br>programmer.<br>creator.<br>dreamer.</p>
                </div>
            </div>
            <div id="aboutImage" class="container">
                <div class="cell">
                    <img src="resources/seabert-temp.jpg" alt="image of Seabert Yuan" width="500" height="650">
                    <br>
                    <label>seabert yuan 2020</label>
                </div>
            </div>
        </section>
        <section id="projectSection" class="pageSections">
            <div class="container">

                <div id="projectTitle" class="cell">
                    <h2>projects</h2>
                </div>
            </div>
            <div class="container">
                <div class="specialCell">
                    <div class="cell">
                        coming soon
                    </div>
                    <div class="cell">
                        coming soon
                    </div>
                    <div class="cell">
                        coming soon
                    </div>
                </div>
            </div>
        </section>
        <section id="contactSection" class="pageSections">
            <div id="contactTitle" class="container">
                <div class="cell">
                    <h2>contact</h2>
                </div>
            </div>
            <div id="inputContainer" class="container">
                <form id="emailForm">
                    <input type="text" id="emailEnter" placeholder="email">
                    <label for="emailEnter">email</label>
                    <input type="text" id="nameEnter" placeholder="name">
                    <label for="nameEnter">name</label>

                    <input type="submit" value="Submit">
                </form>
            </div>
        </section>
    </body>
</html>