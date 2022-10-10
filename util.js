// function loopFrame(fn) {
//   let rafId = null
//   const loop = () => {
//     fn && fn()
//     rafId = window.requestAnimationFrame(loop)
//   }
//   loop()
//   return rafId
// }

function addZeros(number) {
  let output = Math.abs(number).toString()
  while (output.length < 2) {
    output = '0' + output
  }
  return output
}

export { addZeros }
