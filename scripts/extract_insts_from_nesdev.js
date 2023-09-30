// ==UserScript==
// @name        Extract insts from reference - nesdev.org
// @namespace   Violentmonkey Scripts
// @match       https://www.nesdev.org/obelisk-6502-guide/reference.html
// @grant       none
// @version     1.0
// @author      -
// @description 2023/9/30 23:12:47
// ==/UserScript==

console.log("test for nesdev")
var insts_html = document.getElementsByTagName('h3')
var insts_names = Array()
for (var i = 0; i < insts_html.length; i++) {
  insts_names.push(insts_html[i].innerText)  // e.g. ADC - Add with Carry
}
insts_names = insts_names.map(function(x) {
  return x.split(' ')[0]
})
console.log(insts_names)
var tbodys_html = document.getElementsByTagName('tbody')
var insts_lists = Array()
for (var i = 2; i < tbodys_html.length; i += 2) {
  insts_lists.push(tbodys_html[i]);
}

const IDX_ADDR_METHOD = 0
const IDX_OPCODE = 1
const IDX_BYTES = 2
const IDX_CYCLES = 3

function getTRsInnerTextsArray(x) {
  return [x[IDX_ADDR_METHOD].innerText, x[IDX_OPCODE].innerText.replace('$', '0x'), x[IDX_BYTES].innerText, x[IDX_CYCLES].innerText]
}

/* Just for test */
// var x = insts_lists[0]
// var trs = x.getElementsByTagName('tr')
// for (var j = 1; j < trs.length; j++) {
//   var tr = trs[j]
//   var tds = tr.getElementsByTagName('td')
//   console.log(getTRsInnerTextsArray(tds))
// }

var insts = []
insts_lists.forEach(function(x) {
  var trs = x.getElementsByTagName('tr')
  var one_inst_array = Array()
  for (var i = 1; i < trs.length; i++) {
    one_inst_array.push(getTRsInnerTextsArray(trs[i].getElementsByTagName('td')))
  }
  insts.push(one_inst_array)
})
console.log(insts)

var address_method_dict = {
  'Immediate': '',
  "Zero Page": '',
  "Zero Page,X": '',
  "Absolute": '',
  "Absolute,X": '',
  "Absolute,Y": '',
  "(Indirect,X)": '',
  "(Indirect),Y": ''
}

for (var i = 0; i < insts.length; i++) {
  var inst_name = insts_names[i];
  var inst = insts[i]
  inst.forEach(function(x) {
    console.log(x[IDX_OPCODE] + '=>{' + 'Instruction::' + inst_name + '(opcode, ADDRESSING_METHOD, OPERAND_SINGLE_TYPE)}')
  })
}