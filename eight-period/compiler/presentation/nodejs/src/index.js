const { open, convertToArray, write  } = require('./file')
const { pipe, getRandomFromArr } = require('./util')

exports.main = () => {
  const convertFile = pipe(open, convertToArray)
  const names = convertFile('names.csv')
  const pronomes = convertFile('pronomes.csv')

  const createUser = pipe(generateFullName(pronomes), generateEmail, convertToCsv)

  const users = names.map(createUser)

  write('output.csv', users.join('\n'))
  console.log('File has been created and written')
}

const generateEmail = fullName => {
  const providers = convertToArray(open('providers.csv'))
  const provider = getRandomFromArr(providers)

  const email = `${fullName[0]}.${fullName[1]}@${provider}`

  return [ ...fullName, email ]
}

const generateFullName = pronomes => name => {
  const randomLastName = getRandomFromArr(pronomes)
  return [name, randomLastName]
}

const convertToCsv = user => {
  return user.join(';')
}
