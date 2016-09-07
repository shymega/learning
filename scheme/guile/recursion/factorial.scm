;;; Factorial function in Scheme (GNU Guile)

(define (factorial n)
  (cond
   ((zero? n) 1)
   (#t (* n
          (factorial
           (- n 1))))))
