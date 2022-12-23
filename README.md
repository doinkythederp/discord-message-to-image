# Discord Message -> Image

Little experiment for making pngs of discord images.

## How to use it

You'll need to have Node.js (for the webserver), pnpm (to install Node.js packages), and Cargo/Rust (for controlling the browser) installed.

### Setting up a web driver

Having to deal with rendering and wrapping text is kind of a pain so I just used HTML/CSS. Unfortunately, this means you also need an automated web browser and a website (see next step) running in the background. There are a few ways to get this set up, choose one:

#### Option 1 - Safari + safaridriver (recommended for Mac)

If you're on a Mac and have admin access this will be the easiest option. Open a terminal and type `safaridriver --enable` to enable automation. Enter your password if it's asked for. Then type `safaridriver -p 4444` to start the web driver.

#### Option 2 - Chrome + chromedriver (recommended for non-Mac)

Make sure Google Chrome is installed and then download chromedriver from <https://chromedriver.chromium.org/downloads>. Open up the unzipped folder in a terminal and type `./chromedriver --port 4444`.

#### Option 3 - Firefox + geckodriver (alternative)

I'm not totally sure how this one works but it's probably pretty similar to the Google Chrome one.

### Starting the website server

Open up the `web` folder in a new terminal tab (web driver still running!) and type `pnpm install && pnpm dev`. This will start the web server on port 3000.

### Running the actual program

Open up this folder in a new terminal tab (last two things still running!) and type `cargo run` to start the CLI tool. Follow the instructions and your channel will be saved as `output.png`.
