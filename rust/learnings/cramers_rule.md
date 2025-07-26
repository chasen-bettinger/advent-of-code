What does it mean for something to be Collinear
⎿ Found 8 new diagnostic issues in 1 file (ctrl-r to expand)

⏺ Collinear means "lying on the same straight line."

Visual Examples

Collinear points:
A ---- B ---- C ---- D
(All points lie on one straight line)

Non-collinear points:
A ---- B
|
C
(Points form an angle, not a straight line)

In Mathematics

Three points A, B, C are collinear if:

- The slope from A to B equals the slope from B to C
- OR: The area of triangle ABC = 0
- OR: Vector AB is parallel to vector AC

In Your Claw Machine Context

If Button A and Button B movements are collinear, it means:
// If these vectors are collinear:
button_a = (94, 34)
button_b = (22, 67)

// Check: 94/22 should equal 34/67
94/22 ≈ 4.27
34/67 ≈ 0.51
// Not equal → NOT collinear

Why it matters: If buttons are collinear, you can only reach points along that single line - making
some prizes impossible to reach. Non-collinear buttons give you 2D movement freedom.

> @src/puzzle_13.rs#L114-121 Can you visually explain this
> algorithm?
> ⎿ Read src/puzzle_13.rs (8 lines)
> ⎿ ⧉ Selected 8 lines from src/puzzle_13.rs in Visual

    Studio Code

⏺ This algorithm solves a system of linear equations using Cramer's Rule. Let me break it down
visually:

The System of Equations

We want to solve:
x1 _ a + y1 _ b = z1 (equation 1)
x2 _ a + y2 _ b = z2 (equation 2)

For the claw machine:
button_a.x _ a + button_b.x _ b = prize.x
button_a.y _ a + button_b.y _ b = prize.y

Visual Example

Button A: (94, 34) Button B: (22, 67) Prize: (8400, 5400)

94*a + 22*b = 8400 (X equation)
34*a + 67*b = 5400 (Y equation)

Parameters:
x1=94, y1=22, z1=8400
x2=34, y2=67, z2=5400

Algorithm Steps

Step 1: Solve for b (Line 115)
b = (z2 _ x1 - z1 _ x2) / (y2 _ x1 - y1 _ x2)
b = (5400 _ 94 - 8400 _ 34) / (67 _ 94 - 22 _ 34)
b = (507600 - 285600) / (6298 - 748)
b = 222000 / 5550 = 40

Step 2: Solve for a (Line 116)
a = (z1 - b _ y1) / x1
a = (8400 - 40 _ 22) / 94
a = (8400 - 880) / 94
a = 7520 / 94 = 80

Step 3: Verify Solution (Lines 117-119)
Check: (94*80 + 22*40, 34*80 + 67*40) = (8400, 5400) ✅
If exact match → return cost: 80\*3 + 40 = 280
If no exact match → return 0 (no solution)

This is much faster than brute force because it directly calculates the answer using math
instead of trying all combinations!

> What books should I read to learn about this and similar strategies?

⏺ Here are the best books to learn these mathematical problem-solving strategies:

Linear Algebra & Systems of Equations

- "Linear Algebra and Its Applications" by Gilbert Strang
  - The gold standard for understanding systems of equations, matrices, and Cramer's rule
- "Introduction to Linear Algebra" by Gilbert Strang
  - More accessible version, perfect for programming applications

Algorithms & Mathematical Optimization

- "Introduction to Algorithms" (CLRS) by Cormen, Leiserson, Rivest, Stein
  - Chapter on linear programming and mathematical foundations
- "Algorithm Design" by Jon Kleinberg and Éva Tardos
  - Excellent for understanding when to use math vs brute force

Competitive Programming (Advent of Code style)

- "Competitive Programming 4" by Steven Halim
  - Covers exactly these types of optimization problems
- "Guide to Competitive Programming" by Antti Laaksonen
  - Great for learning pattern recognition in contest problems

Mathematical Problem Solving

- "Concrete Mathematics" by Graham, Knuth, Patashnik
  - The bible for mathematical techniques in computer science
- "How to Solve It" by George Pólya
  - Classic on mathematical problem-solving strategies

Discrete Mathematics

- "Discrete Mathematics and Its Applications" by Kenneth Rosen
  - Foundation for understanding mathematical structures in programming

Start with Strang's "Introduction to Linear Algebra" and Kleinberg's "Algorithm Design" -
they'll give you the exact tools used in this puzzle!
