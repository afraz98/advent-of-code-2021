const path = require('path');
const fs = require('fs');
const input = fs.readFileSync(path.join(__dirname, 'sonar_sweep.txt'), 'utf8').toString().trim().split('\n').map((num) => parseInt(num, 10));

const countIncreasedMeasurements = (measurements) => {
    let count = 0;
    console.log(measurements)
    for(let i = 1; i < measurements.length; i++){
        if(measurements[i] > measurements[i-1]){
            count++;
        }
    }

    return count;
}

const countIncreasedSums = (measurements) => {
    let sums = []
    for(let i = 0; i < measurements.length - 2; i++){
        sums.push(measurements[i] + measurements[i+1] + measurements[i+2])
    }

    return countIncreasedMeasurements(sums)
}

console.log(countIncreasedMeasurements(input))
console.log(countIncreasedSums(input))