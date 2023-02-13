// Esta es la forma tradicional de declarar una variable
var lastName = "Duque";
var lastName = "Oscar";
console.log(lastName);

// Let llegó con la actualización de ES6, es más segura para crear variables
let fruit = "Apple";
fruit = "Kiwi";
console.log(fruit);

// Const también llegó con ES6, permite crear constantes
const animal = "Dog";
animal = "Cat";
console.log(animal);

// Ésta es la sintaxis básica para crear una función flecha (arrow function)
const fruits = () => {
    if(true) {
        var fruit1 = "Apple";
        let fruit2 = "Kiwi";
        const fruit3 = "Durazno";
    }
    console.log(fruit1);
    console.log(fruit2);
    console.log(fruit3);
};

fruits();

// Las arrow function llegaron con ES6, permite ahorrar líneas de código
const square = (num) => {return num * num};

/* Cuando en una arrow function solo tengamos 1 parámetro, podemos omitir
paréntesis y llaves, el return se colocará de manera explícita */
const doble = num => num * num;