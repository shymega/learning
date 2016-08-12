(define-macro my-or
  (lambda (x y)
    '(let (temp ,x))
    (if temp temp ,y)))

(define-macro my-or
  (lambda (x y)
    '(let (temp ,x))
    (if temp temp ,y)))


(define-macro my-or
  (lambda (x y)
    `(let ((temp ,x))
       (if temp temp ,y))))

(define temp 3)

(my-or #f temp)
