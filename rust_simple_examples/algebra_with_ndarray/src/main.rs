mod types;
fn main() {
    types::matrix::print_matrix();
    types::matrix::print_matrix_ones();
    types::matrix::print_matrix_zeros();
    types::matrix::print_matrix_rand();
    types::matrix::print_matrix_identity();
    types::vector::print_vector();

    types::matrix::print_matrix_addition();
    types::matrix::print_matrix_scalar_mul_div();
    types::matrix::print_matrix_dot_product();
    types::matrix::print_matrix_transpose();

    types::matrix::print_matrix_info();
    types::matrix::print_matrix_index();
    types::matrix::print_matrix_manipulation();
}
