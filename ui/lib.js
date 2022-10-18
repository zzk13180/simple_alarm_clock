class Digital {
  constructor() {
    this.elements = {
      hour: document.querySelector('#digital-hours'),
      minute: document.querySelector('#digital-minutes'),
      second: document.querySelector('#digital-seconds'),
    }
    this.second = null
    this.minute = null
    this.hour = null
    return new Proxy(this, {
      set: function (target, key, value, receiver) {
        const oldValue = target[key]
        const result = Reflect.set(target, key, value)
        if (!Object.is(value, oldValue)) receiver.trigger(key, value)
        return result
      },
    })
  }
  trigger(key, value) {
    this.elements[key].textContent = value
  }
}

export { Digital }
