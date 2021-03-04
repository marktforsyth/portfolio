shape = [
  ['a', 'a', 'a', 'a'],
  ['a', ' ', ' ', 'a'],
  ['a', ' ', ' ', 'a'],
  ['a', 'a', 'a', 'a']
]

def draw(shape):
  for line in shape:
    print(' '.join(line))

draw(shape)