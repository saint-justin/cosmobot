import { ScrapeRequest } from "./types";
import { writeFile } from "fs";

const req_url = "https://marvelsnapzone.com/getinfo/?searchtype=cards&searchcardstype=true";

let run = async () => {
    let req = await fetch(req_url);
    let res = await req.json() as ScrapeRequest;
    console.log("Total cards: " + res.success.cards.length)
    writeFile("../snap-data/cards.json", JSON.stringify(res.success), e => {
        if (e) {
            console.log(e);
        }
    })
}

run();