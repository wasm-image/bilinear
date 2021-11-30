import { bilinear as bilinearWASM } from "../pkg/index.js";
import bilinearJS from "./lib/bilinear";

const benchmark = (image, w, h) => {
  const suite = new Benchmark.Suite();

  suite
    .add("bilinear-js", () => {
      bilinearJS(image, w, h);
    })

    .add("bilinear-wasm", () => {
      bilinearWASM(image, w, h);
    })
    .on("cycle", function (event) {
      console.log(String(event.target));
    })
    .on("complete", function () {
      console.log("Fastest is " + this.filter("fastest").map("name"));
    })
    .run({ async: true, minSamples: 1000 });
};

const main = async () => {
  try {
    const response = await fetch("https://images.unsplash.com/photo-1638189957940-05a99b857142");

    const blob = await response.blob();

    const url = URL.createObjectURL(blob);

    let img = document.getElementById("src-img");
    img.src = url;

    img.onload = () => {
      const canvas = document.createElement("canvas");
      const context = canvas.getContext("2d");

      const { naturalHeight, naturalWidth } = img;

      const scaledWidth = naturalWidth / 4;
      const scaledHeight = naturalHeight / 4;

      canvas.width = naturalWidth;
      canvas.height = naturalHeight;
      context.drawImage(img, 0, 0);
      const data = context.getImageData(0, 0, naturalWidth, naturalHeight);

      const dcanvas = document.querySelector("canvas");

      dcanvas.width = scaledWidth;
      dcanvas.height = scaledHeight;

      const ctx = dcanvas.getContext("2d");

      console.log(data.data);

      ctx.putImageData(bilinearWASM(data, scaledWidth, scaledHeight), 0, 0);

      benchmark(data, scaledWidth, scaledHeight);
    };
  } catch (error) {
    console.error(error);
  }
};

main();
