;; The first program

(begin
  (display "Hello, World!")
  (newline))

(define (hello name)
  (format #f "Hello, ~a! How are you?" name))

(hello "Dom")
