// Smaller size for move struct (u16 is sufficient)
// Pawns moves can be calculated with << 8 and attacks with <<7 and <<9
// Look at pins and blockers for king
// Generate geometry for each square (diagonal or files and ranks)
// #[inline] and use <const KIND> and stuff for compilation benefits
// compile for CPU
// inline make and undo functions
// copy state instead of undo
