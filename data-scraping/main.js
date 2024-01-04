var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
import fetch from 'node-fetch';
const req_url = "https://marvelsnapzone.com/getinfo/?searchtype=cards&searchcardstype=true";
console.log("Fetching JSON blob...");
let call = () => __awaiter(void 0, void 0, void 0, function* () {
    let req = yield fetch(req_url);
    let res = yield req.json();
    console.log(res.GET);
});
call();
