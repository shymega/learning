(define display3
  (lambda (arg1 arg2 arg3)
    (display arg1)
    (display " ")
    (display arg2)
    (display " ")
    (display arg3)
    (display " ")
    (newline)))

(display3 2 "t" #f)

(cond ((char<? #\d #\c) -1)
      ((char=? #\d #\c) 0)
      (else 1))
