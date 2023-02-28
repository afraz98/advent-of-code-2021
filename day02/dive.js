const path = require('path');
const fs = require('fs');
const input = fs.readFileSync(path.join(__dirname, 'test_dive.txt'), 'utf8').toString().trim().split('\n');

const dive = (input) => {
    let depth = 0;
    let horizontal = 0;

    input.map (
        (line) => {
            let value = line.split(" ");
            if(value[0] === "forward"){
                horizontal += parseInt(value[1], 10)
            } else if(value[0] === "up"){
                depth -= parseInt(value[1], 10)
            } else if(value[0] === "down"){
                depth += parseInt(value[1], 10)
            }
        }
    )

    return depth * horizontal;
}

console.log(dive(input));

const dive_with_aim = (input) => {
    let depth = 0;
    let horizontal = 0;
    let aim = 0;
    input.map (
        (line) => {
            let value = line.split(" ");
            if(value[0] === "forward"){
                horizontal += parseInt(value[1], 10)
                depth += parseInt(value[1], 10) * aim
            } else if(value[0] === "up"){
                aim -= parseInt(value[1], 10)
            } else if(value[0] === "down"){
                aim += parseInt(value[1], 10)
            }
        }
    )

    return depth * horizontal;
}

console.log(dive_with_aim(input))