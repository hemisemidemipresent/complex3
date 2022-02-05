// window.init().then(() => {
//     let multiplier = 1;
//     var startTime = performance.now();

//     let res = window.evaluate('z^3/100+1+i', multiplier);
//     var endTime = performance.now();
//     let posLen = 3 * Math.pow(20 * multiplier + 1, 2);

//     let pos = res.slice(0, posLen);
//     let colors = res.slice(posLen);

//     console.log(`Call to evaluate took ${(endTime - startTime) / 1000} s`);
//     console.log(Array.from(res));
//     console.log(Array.from(pos));
//     console.log(Array.from(colors));
// });
