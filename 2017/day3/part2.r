# pseudocode
# start with a matrix of dimension 3x3, initialized to [[0,0,0],[0,1,0],[0,0,0]]
# start at position 1,1, and initialize it to 1

embiggen <- function(mat) {
    width <- dim(mat)[1]
    new_width = width + 2

    # I don't know how to use the reshape features in R, so doing this the dumb way
    new_mat = array(0, dim=c(new_width, new_width))

    for(row in 1:width) {
        for(col in 1:width) {
            new_mat[col+1, row+1] = mat[col, row]
        }
    }

    new_mat
}

convolve <- function(row, col, arr) {
    # We assume that the array is embiggened enough that there are no edge conditions to deal with
    arr[row-1,col-1] + arr[row-1,col] + arr[row-1,col+1] + arr[row,col-1] + arr[row,col+1] + arr[row+1,col-1] + arr[row+1,col] + arr[row+1,col+1]
}

coords_for_shell <- function(shell) {
    # assuming we're in an arraythat's one shell larger than the shell we're considering
    # shell n has a diameter of 2n-1
    diam <- shell * 2 - 1

    # pre-initialize our matrix of indexes to the right size
    indices <- matrix(0, nrow=4*(diam-1), ncol=2)

    # start at one empty shell plus the diameter
    row <- diam + 1
    col <- diam + 1

    index <- 1

    # take the (diam - 1) above the starting point
    for(i in 1:(diam-1)) {
        row <- row - 1
        indices[index, 1] <- row
        indices[index, 2] <- col
        index <- index + 1
    }

    # from there go left
    for(i in 1:(diam-1)) {
        col <- col - 1
        indices[index, 1] <- row
        indices[index, 2] <- col
        index <- index + 1
    }

    # from there go down
    for(i in 1:(diam-1)) {
        row <- row + 1
        indices[index, 1] <- row
        indices[index, 2] <- col
        index <- index + 1
    }

    # from there go right
    for(i in 1:(diam-1)) {
        col <- col + 1
        indices[index, 1] <- row
        indices[index, 2] <- col
        index <- index + 1
    }

    indices
}


find <- function(sentinel) {
    mat = array(c(0,0,0,0,1,0,0,0,0,0), dim=c(3,3))

    start_row <- 1
    start_col <- 1
    shell <- 1
    latest <- 1

    repeat {
        shell <- shell + 1
        mat <- embiggen(mat)
        coords <- coords_for_shell(shell)
        num_coords <- length(coords)/2

        for(index in 1:num_coords) {
            row <- coords[index, 1]
            col <- coords[index, 2]
            new_val <- convolve(row, col, mat)
            mat[row, col] <- new_val
            if(new_val > sentinel) {
                return(new_val)
            }
        }
    }
}

print(find(361427)) # hard-coded sentinel based on my Advent of Code input
