exports.pipe = (...fns) => {
  const firstFn = fns.shift()
  return (...content) => fns.reduce((acc, fn) => fn(acc), firstFn(...content))
}

exports.getRandomFromArr = arr => arr[Math.floor(Math.random() * arr.length)]

