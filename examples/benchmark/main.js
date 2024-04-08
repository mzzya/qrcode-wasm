var QRCode = require('qrcode')

var qr = require('@mzzya/qrcode-wasm')

var fs = require('fs')

var urls = new Array(100).fill(0).map((m, idx) => `https://item.jd.com/100078858166.html`)

// 计时器
const timer = async (func) => {
    const start = new Date().valueOf()
    console.log(`计时器开始`)
    const res = await func
    const dur = new Date().valueOf() - start
    console.log(`计时器结束，耗时(毫秒):`, dur)
    return res
}


const nodeQrCode = async () => {
    const res = []
    for (let i = 0; i < urls.length; i++) {
        const url = urls[i];
        const str = await QRCode.toDataURL(url, { type: 'png', width: 200 })
        res.push(str)
    }
    return res;
}

const wasmQrCode2 = async () => {
    const res = []
    const ops = new qr.Options()
    
    for (let i = 0; i < urls.length; i++) {
        const url = urls[i];
        const str = await qr.qr_code(url, ops)
        res.push(str)
    }
    return res;
}
const main = async () => {
    const strs1 = await timer(nodeQrCode())

    // console.log("strs1", strs1[0])
    // console.log("strs1", JSON.stringify(strs1))
    // fs.writeFileSync('./strs1.png', strs1[0])

    const strs3 = await timer(wasmQrCode2())
    // console.log("strs3", JSON.stringify(strs3))
    // fs.writeFileSync('./strs3.png', strs3[0])
}
main()