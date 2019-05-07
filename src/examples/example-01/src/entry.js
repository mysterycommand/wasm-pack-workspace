import { run } from "./lib.rs";

const canvas = document.createElement("canvas");
const ctx = canvas.getContext("2d");

console.log(ctx);
run(ctx);
