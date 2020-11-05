const fs = require('fs')
const path = require('path')

exports.open = filename => {
  const file = path.join(__dirname, '..', '..', filename)
  return fs.readFileSync(file)
}

exports.write = (filename, data) => {
  fs.writeFileSync(filename, data);
}

exports.convertToArray = fileContent => {
  const arr = fileContent.toString().split('\n')
  arr.pop()
  return arr
}

