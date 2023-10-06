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
  'Immediate': 'self._resolve_imm_opnd(memory)',
  "Zero Page": 'self._resolve_zero_page_opnd(memory)',
  "Zero Page,X": 'self._resolve_zero_page_x_opnd(memory)',
  "Zero Page,Y": 'self._resolve_zero_page_y_opnd(memory)',
  "Absolute": 'self._resolve_absolute_opnd(memory)',
  "Absolute,X": 'self._resolve_absolute_x_opnd(memory)',
  "Absolute,Y": 'self._resolve_absolute_y_opnd(memory)',
  "(Indirect,X)": 'self._resolve_index_indirect_opnd(memory)',
  "(Indirect),Y": 'self._resolve_indirect_index_opnd(memory)',
  "Indirect": 'self._resolve_indirect_opnd(memory)',
  "Implied": "0",
  "Accumulator": "0",
  "Relative": "self._resolve_imm_opnd(memory)"
}

var address_method_to_enum = {
  'Immediate': 'OperandType::Imm',
  "Zero Page": 'OperandType::Mem',
  "Zero Page,X": 'OperandType::Mem',
  "Zero Page,Y": 'OperandType::Mem',
  "Absolute": 'OperandType::Mem',
  "Absolute,X": 'OperandType::Mem',
  "Absolute,Y": 'OperandType::Mem',
  "(Indirect,X)": 'OperandType::Mem',
  "(Indirect),Y": 'OperandType::Mem',
  "Indirect": 'OperandType::Indirect',
  "Implied": 'OperandType::Implied',
  "Accumulator": "OperandType::Accumulator",
  "Relative": "OperandType::Relative"
}

var operand_encoding = {
  '3' : 'OPERAND_DOUBLE_ENCODING',
  '2' : 'OPERAND_SINGLE_ENCODING',
  '1' : 'OPERAND_NON'
}

for (var i = 0; i < insts.length; i++) {
  var inst_name = insts_names[i];
  var inst = insts[i]
  inst.forEach(function(x) {
    console.log(x[IDX_OPCODE] + '=>' + 'Instruction::' + inst_name + '(opcode, ' + address_method_dict[x[IDX_ADDR_METHOD]] + ', ' + operand_encoding[x[IDX_BYTES]] + ', ' + address_method_to_enum[x[IDX_ADDR_METHOD]] + ')')
  })
}