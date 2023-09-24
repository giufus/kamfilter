import { FilteredImage } from "kamfilter";
import { memory } from "kamfilter/kamfilter_bg"


var video = document.querySelector("#videoElement");
var canvas_read = document.querySelector("#canvasElementRead");
var canvas_write = document.querySelector("#canvasElementWrite");
var pre = document.querySelector("#pre");

if (navigator.mediaDevices.getUserMedia) {
    navigator.mediaDevices.getUserMedia({ video: true, audio: false })
        .then(function (stream) {
            // send stream to video element
            video.srcObject = stream;

            video.addEventListener('playing', () => {

                var aspect = video.videoHeight / video.videoWidth;
                var wantedWidth = video.videoWidth;   // or use height
                var height = Math.round(wantedWidth * aspect);
                console.log(aspect, wantedWidth, height);

                canvas_read.width = wantedWidth;
                canvas_read.height = height;
                canvas_write.width = wantedWidth;
                canvas_write.height = height;

                let ctx_read = canvas_read.getContext('2d', { willReadFrequently: true });
                let ctx_write = canvas_write.getContext('2d', { willReadFrequently: true });

                const renderLoop = () => {

                    const filteredImage = FilteredImage.new(wantedWidth, height);

                    // draw video on read canvas
                    ctx_read.drawImage(video, 0, 0)

                    // get image data from read canvas
                    let imgData = ctx_read.getImageData(0, 0, wantedWidth, height);

                    // get buffer array from image in read canvas
                    const inputImageData = new Uint8Array(imgData.data.buffer);

                    // apply filter and change cells content 
                    /*filteredImage.invert(inputImageData)
                    const outputImageData = new Uint8Array(memory.buffer, filteredImage.cells(), wantedWidth * height * 4);
                    const outputImage = new ImageData(new Uint8ClampedArray(outputImageData), wantedWidth, height);
                    ctx_write.putImageData(outputImage, 0, 0);
                    */
                   
                    // print chars to pre
                    filteredImage.art(inputImageData)
                    const chars_size_ptr = filteredImage.chars_size();
                    const chars_size = new Uint32Array(memory.buffer, chars_size_ptr, 2);

                    const x = chars_size[0]
                    const y = chars_size[1]

                    const chars_ptr = filteredImage.chars();
                    const chars = new Uint8Array(memory.buffer, chars_ptr, filteredImage.chars_length());
                    
                    let s = ""
                    for (let i = 0; i < y; i++) {
                        for (let j = 0; j < x; j++) {
                            const index = i * x + j
                            const c = chars[index]
                            s += String.fromCharCode(c)
                        }       
                    }
                    pre.innerHTML = s

                    requestAnimationFrame(renderLoop)
                }

                
                requestAnimationFrame(renderLoop)
            });


        })
        .catch(function (err0r) {
            console.log("Something went wrong!", err0r);
        });
}