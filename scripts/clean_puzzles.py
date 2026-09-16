import json
import collections
import random

def calc_d(s1, s2):
    c1 = collections.Counter(s1)
    c2 = collections.Counter(s2)
    diff = 0
    all_chars = set(c1.keys()) | set(c2.keys())
    for char in all_chars:
        diff += abs(c1[char] - c2[char])
    return diff

def generate_source(answer):
    chars = list(answer)
    n = len(chars)
    while True:
        i, j = random.sample(range(n), 2)
        # Pick two random uppercase letters that are NOT at those positions and don't reduce D
        # To be safe, pick letters not in the multiset at all if possible
        available = [chr(c) for c in range(ord('A'), ord('Z')+1)]
        n1 = random.choice(available)
        n2 = random.choice(available)

        temp = chars[:]
        temp[i] = n1
        temp[j] = n2
        if calc_d("".join(temp), answer) == 4:
            random.shuffle(temp)
            return "".join(temp)
            
# Replace with the actual path
path = "<path-to-project>/Lexivo/assets/data/puzzles-medium.json"

with open(path, 'r') as f:
    data = json.load(f)

fixed = 0
for item in data:
    source = item['source']
    answer = item['answer']
    hint = item['hint']

    invalid = False
    if len(source) != len(answer):
        invalid = True
    elif calc_d(source, answer) != 4:
        invalid = True

    if invalid:
        item['source'] = generate_source(answer)
        fixed += 1

    if "(anagram)" in hint:
        item['hint'] = hint.replace("(anagram)", "").strip()

# print(f"Fixed {fixed} puzzles.")
print(json.dumps(data, indent=2))
# print(f"Fixed {fixed} puzzles.")
