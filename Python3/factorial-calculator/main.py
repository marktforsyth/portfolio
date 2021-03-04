nums = {
    'num': 5,
    'outNum': 1
}

def factorial():
    if nums['num'] > 0:
        nums['outNum'] *= nums['num']
        nums['num'] -= 1
        factorial()
        
factorial()
print(nums['outNum'])