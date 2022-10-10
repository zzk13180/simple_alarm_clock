import 'css3clock.css'
import { Digital } from './lib'
import { addZeros } from './util'

const { invoke } = window.__TAURI__.tauri

const updateWatch = () => {
  const now = new Date()
  const hour = now.getHours()
  const minute = now.getMinutes()
  const second = now.getSeconds()
  const styleEl = document.createElement('style')
  const secondDeg = second * 6
  const minuteDeg = minute * 6 + second * 0.1
  const hourDeg = hour * 30 + minute * 0.5 + second * (0.5 / 60)
  styleEl.innerHTML = `
    :root {
      --seconds-rotate-original: ${secondDeg}deg;
      --seconds-rotate-to: ${360 + secondDeg}deg;
      --minutes-rotate-original: ${minuteDeg}deg;
      --minutes-rotate-to: ${360 + minuteDeg}deg;
      --hours-rotate-original: ${hourDeg}deg;
      --hours-rotate-to: ${360 + hourDeg}deg; /* 360 + hour original */
    }
  `
  document.head.append(styleEl)
}
updateWatch()

const digital = new Digital()
const updateDigital = () => {
  const now = new Date()
  digital.hour = addZeros(now.getHours())
  digital.minute = addZeros(now.getMinutes())
  digital.second = addZeros(now.getSeconds())
}
updateDigital()
setInterval(updateDigital, 1000)
invoke("alarm") // test
