function toCamelCase(str){
  const splittedString = str.split(/-|_/)
  const isFirstWordUpper = splittedString[0].charAt(0) === splittedString[0].charAt(0).toUpperCase()
  
  return splittedString.map((str, i) => {
    if (i === 0 && !isFirstWordUpper) {
        return str.toLowerCase()
    }
    const camelWord = str.split("").map((char, i) => i === 0 ? char.toUpperCase() : char.toLowerCase()).join().replaceAll(",","")
    return camelWord
  }).join().replaceAll(",","")
}
