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
    (for [line (line-seq rdr)
      :let [tower (parse_tower line)]]
      tower
    )
  )
)

(defn in? 
  "true if coll contains elm"
  [coll elm]
  (some #(= elm %) coll)
)

(defn swap [coll left right]
  (concat (take left coll) [(nth coll right)] (take (- right left 1) (drop (+ left 1) coll)) [(nth coll left)])
)

(defn maybe_swap [towers, left, right]
  (def left_tower (nth towers left))
  (def right_tower (nth towers right))
  (if (in? (:children right_tower) (:name left_tower))
    (swap towers left right)
    towers
  )
)

(defn sort_step [towers, left, right]
  (if (and (= (- (count towers) 1) left) (= (count towers) right))
    towers
    (if (= (count towers) right)
      (sort_step (+ left 1) (+ left 2))
      (maybe_swap towers left right)
    )
  )
;  (match [left right]
;         [(- (count towers) 1) (count towers)] towers
;         [_ (count towers)] (sort_step (+ left 1) (+ left 2))
;         :else (maybe_swap towers left right) )
)

(defn topo_sort [towers]
  ; It's not the most efficient, but to topologically sort we can just do a swap sort whenever we find
  ; a dependency when the order is wrong
  (nth towers 1)
)

(def filename "test_input")
(def towers (apply list (load_towers filename)))
;(print (first (topo_sort towers)))

