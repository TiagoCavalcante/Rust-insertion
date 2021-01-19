from libsquare import square

number = int(input('Enter a number to be squared n times: '))
times = int(input('Enter n: '))

result = number

for i in range(times):
	result = square(result)

print (result)