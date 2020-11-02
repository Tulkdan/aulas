const { open, convertToArray  } = require('./file')
const { pipe, getRandomFromArr } = require('./util')

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

exports.main = () => {
  const convertFile = pipe(open, convertToArray)
  const names = convertFile('names.csv')
  const pronomes = convertFile('pronomes.csv')

  const createUser = pipe(generateFullName(pronomes), generateEmail)

  const users = names.map(createUser)

  console.log(users)
}
