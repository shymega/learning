;;; Factorial function in Scheme (GNU Guile)

(define (fact n)
  (cond ((zero? n) 1)
        (#t (* n (fact (- n 1))))))
