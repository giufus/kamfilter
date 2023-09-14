import {process, blur_image_and_draw_from_js, FilteredImage } from "kamfilter";
import { memory } from "kamfilter/kamfilter_bg"



var video = document.querySelector("#videoElement");
var canvas_read = document.querySelector("#canvasElementRead");
var canvas_write = document.querySelector("#canvasElementWrite");


if (navigator.mediaDevices.getUserMedia) {
    navigator.mediaDevices.getUserMedia({ video: true, audio: false })
        .then(function (stream) {
            // send stream to video element
            video.srcObject = stream;

            video.addEventListener('playing', () => {

                var aspect = video.videoHeight / video.videoWidth;
                var wantedWidth = 640;   // or use height
                var height = Math.round(wantedWidth * aspect);
                console.log(aspect, wantedWidth, height);

                canvas_read.width = wantedWidth;
                canvas_read.height = height;

                canvas_write.width = wantedWidth;
                canvas_write.height = height;

                let ctx_read = canvas_read.getContext('2d', { willReadFrequently: true });
                let ctx_write = canvas_write.getContext('2d', { willReadFrequently: true });

                const filteredImage = FilteredImage.new(wantedWidth, height);
                //let imgData = ctx_read.getImageData(0, 0, canvas_read.width, canvas_read.height);
                //console.log(imgData.data);
                //ctx_read.scale(0.6, 0.4)
    

                const renderLoop = () => {
                    
                    // draw video on read canvas
                    ctx_read.drawImage(video, 0, 0)

                    // get image data from read canvas
                    let imgData = ctx_read.getImageData(0, 0, wantedWidth, height);

                    // get buffer array from image in read canvas
                    const inputImageData = new Uint8Array(imgData.data.buffer);
                    
                    // apply filter and change cells content
                    filteredImage.fill_cells(inputImageData)
                    
                    //console.log(filteredImage.width(), filteredImage.height())

                    const outputImageData = new Uint8Array(memory.buffer, filteredImage.cells(), wantedWidth * height * 4);
                    
                    const outputImage = new ImageData(new Uint8ClampedArray(outputImageData), wantedWidth, height);
                    
                    ctx_write.putImageData(outputImage, 0, 0);
                    //ctx_write.fillText(process(""), 50, 100);

                    requestAnimationFrame(renderLoop)
                }

                requestAnimationFrame(renderLoop)
            });


        })
        .catch(function (err0r) {
            console.log("Something went wrong!", err0r);
        });
}