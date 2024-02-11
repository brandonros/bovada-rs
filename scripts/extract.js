#!/usr/bin/env node

const fs = require('fs')

const round = (value, decimals) => {
    return Number(Math.round(value + 'e' + decimals) + 'e-' + decimals)
} 

const main = async () => {
    if (process.argv.length < 4) {
        console.error('Usage: extract.py <EVENT_ID> <OUTCOME_ID>')
        process.exit(1)
    }
    const eventId = process.argv[2]
    const outcomeId = process.argv[3]
    const outputFilename = `./generated/${eventId}-${outcomeId}.tsv`
    const input = await fs.promises.readFile(`./output-${eventId}.tsv`, 'utf8')
    const lines = input.split('\n')
    let output = 'timestamp\timplied_probability\tamerican\thandicap\n'
    let numRows = 0
    for (const line of lines) {
        if (!line.includes('PriceEvent')) {
            continue
        }
        if (!line.includes(outcomeId)) {
            continue
        }
        try {
            const parts = line.trim().split('\t')
            const timestamp = parts[1]
            const event = JSON.parse(parts[3])
            let handicap = 0
            if (event.price.handicap) {
                handicap = parseFloat(event.price.handicap)
            }
            const decimalPrice = parseFloat(event.price.decimal)
            const americanPrice = parseFloat(event.price.american)
            const impliedProbability = round(1 / decimalPrice, 2)
            output += `${timestamp}\t${impliedProbability}\t${americanPrice}\t${handicap}\n`
            numRows += 1
        } catch (err) {
            console.error(err)
        }
    }
    if (numRows === 0) {
        throw new Error('numRows === 0')
    }
    await fs.promises.writeFile(outputFilename, output)
}

main()
