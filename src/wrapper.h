// Note that we might have to define more macros of that sort if new versions remove headers again.
// cf. https://gcc.gnu.org/onlinedocs/cpp/Common-Predefined-Macros.html
#define VRNA_VERSION_GEQ(MAJOR, MINOR, PATCH) (VRNA_VERSION_MAJOR > MAJOR || \
        (VRNA_VERSION_MAJOR == MAJOR && (VRNA_VERSION_MINOR > MINOR || \
        (VRNA_VERSION_MINOR == MINOR && VRNA_VERSION_PATCH >= PATCH))))
#include <ViennaRNA/vrna_config.h>
#include <ViennaRNA/2Dfold.h>
#include <ViennaRNA/2Dpfold.h>
#include <ViennaRNA/alifold.h>
#include <ViennaRNA/ali_plex.h>
#include <ViennaRNA/alphabet.h>
#include <ViennaRNA/boltzmann_sampling.h>
#include <ViennaRNA/centroid.h>
#include <ViennaRNA/cofold.h>
#include <ViennaRNA/combinatorics.h>
#include <ViennaRNA/commands.h>
#include <ViennaRNA/concentrations.h>
#include <ViennaRNA/constraints/basic.h>
#include <ViennaRNA/constraints/hard.h>
#include <ViennaRNA/constraints/ligand.h>
#include <ViennaRNA/constraints/SHAPE.h>
#include <ViennaRNA/constraints/soft.h>
#include <ViennaRNA/datastructures/basic.h>
#include <ViennaRNA/datastructures/char_stream.h>
#include <ViennaRNA/datastructures/hash_tables.h>
#include <ViennaRNA/datastructures/heap.h>
#include <ViennaRNA/datastructures/lists.h>
#include <ViennaRNA/datastructures/stream_output.h>
#include <ViennaRNA/dist_vars.h>
#include <ViennaRNA/dp_matrices.h>
#include <ViennaRNA/duplex.h>
#include <ViennaRNA/edit_cost.h>
#include <ViennaRNA/equilibrium_probs.h>
#include <ViennaRNA/eval.h>
#include <ViennaRNA/fold_compound.h>
#include <ViennaRNA/fold.h>
#include <ViennaRNA/fold_vars.h>
#include <ViennaRNA/grammar.h>
#include <ViennaRNA/heat_capacity.h>
#include <ViennaRNA/inverse.h>
#include <ViennaRNA/io/file_formats.h>
#include <ViennaRNA/io/file_formats_msa.h>
#include <ViennaRNA/io/utils.h>
#ifdef VRNA_WITH_JSON_SUPPORT
# include <ViennaRNA/json.h>
#endif
#include <ViennaRNA/landscape/findpath.h>
#include <ViennaRNA/landscape/move.h>
#include <ViennaRNA/landscape/neighbor.h>
#include <ViennaRNA/landscape/paths.h>
#include <ViennaRNA/landscape/walk.h>
#include <ViennaRNA/Lfold.h>
#include <ViennaRNA/loops/all.h>
#include <ViennaRNA/loops/external.h>
#include <ViennaRNA/loops/hairpin.h>
#include <ViennaRNA/loops/internal.h>
#include <ViennaRNA/loops/multibranch.h>
#include <ViennaRNA/LPfold.h>
#include <ViennaRNA/MEA.h>
#include <ViennaRNA/mfe.h>
#include <ViennaRNA/mfe_window.h>
#include <ViennaRNA/mm.h>
#include <ViennaRNA/model.h>
#include <ViennaRNA/move_set.h>
#include <ViennaRNA/pair_mat.h>
#include <ViennaRNA/params/basic.h>
#include <ViennaRNA/params/constants.h>
#include <ViennaRNA/params/convert.h>
#include <ViennaRNA/params/default.h>
#include <ViennaRNA/params/io.h>
#include <ViennaRNA/part_func_co.h>
#include <ViennaRNA/part_func.h>
#include <ViennaRNA/part_func_up.h>
#include <ViennaRNA/part_func_window.h>
#include <ViennaRNA/perturbation_fold.h>
#include <ViennaRNA/plex.h>
#include <ViennaRNA/plotting/alignments.h>
#include <ViennaRNA/plotting/layouts.h>
#include <ViennaRNA/plotting/naview/naview.h>
#include <ViennaRNA/plotting/probabilities.h>
#include <ViennaRNA/plotting/RNApuzzler/RNApuzzler.h>
#include <ViennaRNA/plotting/RNApuzzler/RNAturtle.h>
#include <ViennaRNA/plotting/structures.h>
#include <ViennaRNA/plotting/utils.h>
#include <ViennaRNA/ProfileAln.h>
#include <ViennaRNA/profiledist.h>
#include <ViennaRNA/ribo.h>
#include <ViennaRNA/RNAstruct.h>
#include <ViennaRNA/search/BoyerMoore.h>
#include <ViennaRNA/sequence.h>
#include <ViennaRNA/snofold.h>
#include <ViennaRNA/snoop.h>
#include <ViennaRNA/stringdist.h>
#include <ViennaRNA/structured_domains.h>
#include <ViennaRNA/subopt.h>
#ifdef VRNA_WITH_SVM
# include <ViennaRNA/svm.h>
# include <ViennaRNA/utils/svm.h>
#endif
#include <ViennaRNA/treedist.h>
#include <ViennaRNA/unstructured_domains.h>
#include <ViennaRNA/utils/alignments.h>
#include <ViennaRNA/utils/basic.h>
#include <ViennaRNA/utils/cpu.h>
#include <ViennaRNA/utils/higher_order_functions.h>
#include <ViennaRNA/utils/strings.h>
#include <ViennaRNA/utils/structures.h>
#include <ViennaRNA/utils/units.h>
#include <ViennaRNA/zscore.h>
// new since 2.4.18:
#include <ViennaRNA/pk_plex.h>
#if VRNA_VERSION_GEQ(2,5,0)
# include <ViennaRNA/pf_multifold.h>
# include <ViennaRNA/subopt_zuker.h>
# include <ViennaRNA/wrap_dlib.h>
#endif
#if VRNA_VERSION_GEQ(2,6,0)
# include <ViennaRNA/params/salt.h>
# include <ViennaRNA/constraints/soft_special.h>
# include <ViennaRNA/datastructures/array.h>
# include <ViennaRNA/datastructures/string.h>
# include <ViennaRNA/mconf.h>
#endif
#if VRNA_VERSION_GEQ(2,7,0)
# include <ViennaRNA/loops/gquad.h>
# include <ViennaRNA/constraints/probing.h>
# include <ViennaRNA/datastructures/sparse_mx.h>
# include <ViennaRNA/utils/log.h>
#endif
// `vrna_config.h` provides version macros which allows us to include
// version-specific header files.
//
// Alternatives to this approach would be `-idirafter`
// cf. https://gcc.gnu.org/onlinedocs/gcc/Directory-Options.html
//
// or `__has_include`
// cf. https://gcc.gnu.org/onlinedocs/cpp/_005f_005fhas_005finclude.html
//
// Both of which would require some slightly ugly tricks (adjusting clang_args,
// splitting `wrapper.h`, providing header stubs, version-specific cargo features, etc.)

// We don't want to expose this macro in the bindings.
#undef VRNA_VERSION_GEQ
