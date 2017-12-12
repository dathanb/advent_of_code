(require '[clojure.java.io :as io])
(require '[clojure.string :as str])

(defstruct tower :name :weight :children)

(defn tower_name [line]
  (nth (first (re-seq #"^([a-z]+)" line)) 1)
)

(defn tower_weight [line] 
  (nth (first (re-seq #"([0-9]+)" line)) 1)
)

(defn reverse_nth [index, coll]
  (nth coll index)
)

(defn tower_children [line] 
  (apply list
    (map (partial reverse_nth 1)
      (rest  ;; the first hit is the name of THIS tower
        (re-seq #"([a-zA-z]+)+" line)
      )
    )
  )
)

(defn parse_tower [line] 
  (struct tower (tower_name line) (tower_weight line) (tower_children line))
)

(defn load_towers [fname]
  (with-open [rdr (io/reader fname)]
    ;(doseq [line (line-seq rdr)]
    ;  (println (parse_tower line))
    ;)
    (apply list
      (for [line (line-seq rdr)
            :let [tower (parse_tower line)]]
        tower
      )
    )
  )
)

(defn in?
  "true if coll contains elm"
  [coll elm]
  (some #(= elm %) coll)
)

(defn swap [coll left right]
  (concat (take left coll) [(nth coll right)] (take (- right left 1) (drop (+ left 1) coll)) [(nth coll left)] (drop (+ right 1) coll))
)

(defn should_swap? [towers, left, right]
  (def left_tower (nth towers left))
  (def right_tower (nth towers right))
  (in? (:children right_tower) (:name left_tower))
)

(defn maybe_swap [towers, left, right]
  (def left_tower (nth towers left))
  (def right_tower (nth towers right))
  (if (in? (:children right_tower) (:name left_tower))
    [(swap towers left right) 0 1]
    [towers left (+ right 1)]
  )
)

(defn sort_step [towers, left, right]
  (do (print (format "%d, %d\n" left right))
    (if (and (= (- (count towers) 1) left) (= (count towers) right))
      towers
      (if (= (count towers) right)
        (sort_step towers (+ left 1) (+ left 2))
        (if (should_swap? towers left right)
          (recur (swap towers left right) left (+ left 1))
          (recur towers left (+ right 1))
        )
      )
    )
  )
)


(defn topo_sort [towers]
  (sort_step towers 0 1)
)

(defn find_root [towers right]
  (do (print (format "%d\n" right))
    (if (= right (count towers))
      (nth towers 0)
      (if (should_swap? towers 0 right)
        (recur (swap towers 0 right) 1)
        (recur towers (+ right 1))
      )
    )
  )
)

(def filename "input")
;(def filename "test_input")
(def towers (apply list (load_towers filename)))
(print (find_root towers 1))

