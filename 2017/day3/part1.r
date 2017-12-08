# each square ends with the square of an odd integer in the lower-right corner
# and the number one to the right of the upper left of each square is the square of an even integer
diameter <- function(n) {
    if ((n %% 2 == 1) && (sqrt(n) == floor(sqrt(n)))) {
        sqrt(n)
    } else if(floor(sqrt(n)) %% 2 == 0) {
        floor(sqrt(n)) + 1
    } else {
        floor(sqrt(n)) + 2
    }
}

walk <- function (start, count) {
    if(start <1) {
        start <- 1
    }
    if ( start == 1 ) {
        return(count)
    }

    diam = diameter(start)
    radius = floor(diam/2)
    max_this_shell <- diam * diam
    max_previous_shell <- (diam-2)*(diam-2)
    prev_diam = diameter(max_previous_shell)
    prev_radius = floor(prev_diam/2)

    bottom_left = max_this_shell
    bottom_center = bottom_left - radius
    bottom_left = bottom_center - radius
    center_left = bottom_left - radius
    top_left = center_left - radius
    top_center = top_left - radius
    top_right = top_center - radius
    center_right = top_right - radius

    if (start > bottom_center) {
        walk(start-1, count+1)
    } else if (start == bottom_center) {
        walk(max_previous_shell-prev_radius, count+1)
    } else if (start >= bottom_left) {
        walk(start+1, count+1)
    } else if (start > center_left) {
        walk(start-1, count+1)
    } else if (start == center_left) {
        walk(max_previous_shell - prev_radius*3, count+1)
    } else if (start >= top_left) {
        walk(start+1, count+1)
    } else if (start > top_center) {
        walk(start-1, count+1)
    } else if (start == top_center) {
        walk(max_previous_shell - prev_radius*5, count+1)
    } else if (start >= top_right) {
        walk(start+1, count+1)
    } else if (start > center_right) {
        walk(start-1, count+1)
    } else if (start == center_right) {
        walk(max_previous_shell - prev_radius*7, count+1)
    } else {
        walk(start+1, count+1)
    }
}
#print("Doing walk(1)")
#print(walk(1, 0))
#print("Doing walk(12)")
#print(walk(12, 0))
#print("Doing walk(23)")
#print(walk(23, 0))
#print("Doing walk(1024)")
#print(walk(1024, 0))
print(walk(361527, 0))
