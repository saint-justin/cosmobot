### Snaps, the Marvel Snap Discord Bot!

Snaps is a discord bot for marvel snap. Before running the bot, you'll need to pull in some data for the bot to handle. To do that:

1. Navigate into the `/data-scraping` folder in your terminal
2. With [NodeJS installed](https://nodejs.org), run `npm install` to install required dependencies
3. Run `npm install ts-node --global` to globally install [TS Node](https://www.npmjs.com/package/ts-node)
4. Run `ts-node main.ts` to generate your `snap-data/cards.json` file for local use 

There's also a Python script for pulling assets (cards/variants/locations) but it's not needed yet. The Python script is made by [vlmaier](https://github.com/vlmaier) on GitHub, [script repo here](https://github.com/vlmaier/marvel-snap-scrapr/).

Starting the bot is simple once you'ved scraped the required data. Simply run `cargo run` in this directory and the bot will spin up on its own.

Any errors should be captured and printed to your terminal. 

#### To Do:

- [x] Near-enough searching
- [x] Card lookup
    - [x] Basic:
        - [x] Card text
        - [x] Card art
        - [x] Card cost
        - [x] Cart power
        - [x] Hide spoilers
        - [ ] Allow spoilers
        - [x] Near-enough options
    - [ ] Variants:
        - [ ] Display all variant images + names
        - [ ] Hide spoilers
    - [ ] Spoilers: 
        - [ ] Card text
        - [ ] Card art
        - [ ] Include variants

- [ ] Artist lookup
    - [ ] Look up card art by artist name
    - [ ] Option to include spoilers
    
- [ ] Discord updates copypasta
    - [ ] Observer pattern slash command to link channel
