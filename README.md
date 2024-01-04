### Snaps, the Marvel Snap Discord Bot!

Snaps is a discord bot for marvel snap. To use it, you'll first need to scrape some data. To do that, run the Python script at `./data-scraping/scrapr.py`. That will pull all the data required to run the app and format it into a series of images and JSON blobs that will be read by the bot.

The script is made by [vlmaier](https://github.com/vlmaier) on GitHub, [script repo here](https://github.com/vlmaier/marvel-snap-scrapr/)

Starting the bot is simple once you'ved scraped the required data. Simply run `cargo run` in this directory and the bot will spin up on its own.

Any errors should be captured and printed to your terminal. 

#### To Do:

- Near-enough searching
- Card lookup
    - Basic:
        - Card text
        - Card art
        - Hide spoilers
        - Near-enough options
    - Variants:
        - Display all variant images + names
        - Hide spoilers
    - Spoilers: 
        - Card text
        - Card art
        - Include variants

- Artist lookup
    - Look up card art by artist name
    - Option to include spoilers
    
- Discord updates copypasta
    - Observer pattern slash command to link channel
