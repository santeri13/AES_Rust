# AES_Rust
AES Encription function with predefined message and key

##Description
This program encrypt predefined matrix with predefined key k10 with 10 rounds, because of data all values would be 128bits.

This program contain values:
 - key: this is 128bits encryption key which is suitable for round 10 message
 - message: message contain 128bits matrix

Function which program contain:
 - byte_sub: One of encryption methods in AES. It replace values from message with values from predefined table.
 - shift_row: One of encryption methods in AES. This function shift each row of message. Each time shift value increse by 1. Value start from 0 whcih means that first row would not be shifted.
 - mix_columns: One of encryption methods in AES. This function multiple message with predefined table. For that function was used libary in isochronous_finite_fields for right multiplication of values.
 - inverse_key_expansion: Because our key is k10 program need to reverse k until it become k0.
 - main: activate all functions.

##Byte_Sub

When function recive message it goes into for loop. There from message each time would be taken hex value, becasue in matrix we have 16 values loop would go 16 time.
Each time we split our hex value and check characters which it contain and define coordinates, which would be used for taking specific hex value from S-box.
Next this values would be taken to new_message value and later place in actual message.

##Shift_Row

Funcion take messsage and split each row of messsage by shift value, first row would be 0, second would be 1 and etc. When all rows are shifted programm combine all value into message.

##Mix_Columns:

When function recive message it would be splited for 4 row. Funtion take each row of predefined matrix and each column of message and multiple them. 
With for cycle we take each row of predefined message and next for would construct column from message. Next we need to multiply each hex from both values and xor them: 
hex_value_column_1 * hex_value_row_1 (xor) hex_value_column_2 * hex_value_row_2 (xor) hex_value_column_3 * hex_value_row_3 (xor) hex_value_column_4 * hex_value_row_4.
For multipliction of this values need to use Finite field arithmetic. For that was used one of rust libaries and small code from webpage: https://medium.com/asecuritysite-when-bob-met-alice/galois-field-gf-2%E2%81%B8-and-rust-503aa218f476.
To know that I undertand how it works I make example: we have two value 0001 0000 and 01100011, multiplication of this values: X4 * (1+X+X5+X6) = X4+X5+X6+X7 = 1111 0000.
When we multipled all this values we xor them and put each value into new message whcih would be replace old.

##Inverse_Key_Expansion 
Because our key is k10 program need to reverse k until it become k0. For that we take k10 and make reverse it back by one, so to k9 but in main function we repeat this by 10 time.
Standard Key Expansion take first column of matrix and xor it with last colum of g(matrix). This g use Byte_Sub for colum and next each each row is xored by previouse new matrix value and curent matrix value.
