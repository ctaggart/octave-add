#include <octave/oct.h>

extern octave_value_list Fadd (const octave_value_list&, int);

extern "C" octave_function * Gadd (const octave::dynamic_library& shl, bool relative) {
  octave_dld_function *fcn = octave_dld_function::create (Fadd, shl, "add", "adds inputs and returns sum to all outputs");
  if (relative) fcn->mark_relative ();
  return fcn;
}

octave_value_list Fadd (const octave_value_list& args, int nargout)
{
  int nargin = args.length();
  octave_value_list retval(nargout);
  int sum = 0;
  for (int i = 0; i < nargin; i++)
    sum += args(i).int_value();
  for (int i = 0; i < nargout; i++)
    retval(i) = octave_value(sum);
  return retval;
}