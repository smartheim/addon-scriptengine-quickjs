# addon-scriptengine-quickjs

> OHX is a modern smarthome solution, embracing technologies like software containers for language agnostic extensibility. Written in Rust with an extensive test suite, OHX is fast, efficient, secure and fun to work on.

This is an OHX Addon. It provides a ES2020 JS engine based on QuickJS for rule engine scripts.

* Install this Addon to your local OHX installation via the [OHX CLI](#cli): `ohx-cli addon install addon-scriptengine-quickjs`.
* The maintainer can deploy a new version (after updating `addon.yml`) with `ohx-addon-cli deploy`.

## About this JS Engine

The script engine is **not** NodeJS and of course does not have browser context objects like "window" or "document".
Modules that are covered by the ECMA standard like "Math", "Regex" and "JSON" are available.
Other functions are available globally (if enabled, see [Security and Privacy Options](#security-and-privacy-options)).

* `print(...args)` / `console.log(...args)`
	Print the arguments separated by spaces and a trailing newline.

* `urlGet(url)`
	Download url. The response is returned if the status is between 200 and 299.
	Otherwise an std.Error exception is raised.
    A fixed timeout of 4 seconds is set.

* `sleep(delay_ms)`: Sleep for delay_ms milliseconds.

* `setTimeout(func, delay) -> handle`: Call the function func after delay ms. Return a handle to the timer.

* `clearTimeout(handle)`: Cancel a timer.

* `globalThis.output`: The output variable. As soon as all promises are resolved and the runtime finished
  executing your script, this variable will be used as output. Example usage: `globalThis.output = 12`.

QuickJS compiles scripts down to bytecode and in successive calls execute the bytecode directly.
This should be pretty fast and allows for a invocation-to-return time of about 10ms
in successive calls. 

## Multiple files

You can split your script into multiple files. A script is assumed to be a valid JS Module file.
Use `import * from 'your_other_file.js` to import another module. Backwards references ".." are not allowed,
subdirectories are however.

## JS OHX API in scripts

The following global functions are available to interact with OHX.

Script context:

* `ruletype() -> String`: Returns either "condition" or "action" or "transformation".


For global variables and parameter like variables:

* `setGlobalVar(var_name, var_value)`: Sets a global variable (if "Allow Global Variables" is enabled for this script)
* `getGlobalVar(var_name) -> JSValue`: Gets a global variable (if "Allow Global Variables" is enabled for this script)
* `setNamedOutputValue(name, value:JSValue)`: The output of a script is stored in the implicitely defined "output" name, available for
  following actions in a rule. Sometimes you want to pass a few more values to the next action or next script. This function allows to
  set named values. This is like rule scope local variables. You can only pass simple types: Numbers, Strings, booleans.
* `getNamedInputValue(name) -> JSValue`: Events, Conditions or pre-executed Actions might have set named output values.
  You can access those via this method.

Interact with Things:

* `execThingAction(thing_id, action_id, [arguments]) -> bool`: Executes an addon action. Returns true if the addon
  referenced by `addon_id` accepted the `action_id` and given optional arguments. Returns false otherwise.
  The return value does not express if the action could be completed by the addon or failed unconditionally.

Interact with Thing States:

* `getThingState(thing_id, state_name, state_instance:int) -> JSValue`: Gets a thing state.
  Result might be `null` if the state doesn't exist.
* `getAllThingStates(thing_id) -> [{name,instance,value}]`: Returns an array with all thing states for a given thing id.
  The array contains objects with `name` (state name), `instance` (state instance, number) and `value` attributes.
* `cmdThingState(thing_id, state_name, state_instance:int, value:JSValue) -> bool`: Commands a thing to apply a new state value.
  Depending on the device an Addon might be able to apply a value instantly or over time. Returns true if the Addon that manages the
  referenced `thing_id` accepted the `state_name`, `state_instance` and given value. Returns false otherwise.
  The return value does not express if the value could be applied by the addon.

Observe Thing States:

* `notifyOnThingStatesChange(thing_id, callback:(thing_id, state_name, state_instance:int, state_value:JSValue)->Bool)-> listener_id:int`:
  Registers a lister on thing state changes. Return true in the given callback for further notifications and false to unregister
  the listener. The return value is a listener id that can be used by the next method.
  You can use a javascript function or arrow function as callback.
* `unregisterThingStateListener(listener_id:int)`: Unregister a lister given by the listener id.
  The callback method of a `notifyOnThingStatesChange` will no longer be called.

## Script header

A script can be used as *condition*, *action* and *transformation*.
For a script to be usable in the rule editor, the rule engine must know the scripts possible types, but also a few more other details.
Add those to the script header, which is a single line comment near the front of your script
beginning with "ABOUT:" followed by a json object:

```js
// ABOUT: {'title':'A short title','desc': 'A descriptive text', 'types':["condition", "action"]}
```

## Scripted rule conditions

A *condition* rule script must return with a boolean value, for example:

```js
let abc = getNamedInputValue("abc");
let r = heavy_computation(abc);
r==18
```

## Named Outputs / Inputs

The output of a script is stored in the implicitely defined "output" name, available for
following actions in a rule. "output" is limited to be a single Number, String or boolean however.

Sometimes you want to pass a few more values to the next action or next script.
Named outputs / inputs are like rule scope local variables.

If you want the graphical rule editor to show your named output and input variables, add a single line comment near the the front of your script
beginning with "OUTPUTS:" / "INPUTS:" followed by a json array:

```js
// OUTPUTS: [{'name':'named_output_name','desc': 'A descriptive text', 'example_value':3}]
// INPUTS: [{'name':'named_input_name','desc': 'A descriptive text', 'example_value':3, 'required': false}]
```

The `example_value` is required to determine the output / input type.
The graphical editor forbids to connect outputs to inputs of different type.

## Security and Privacy Options

* **"Allow global variables"**: Global variables are not persisted on disk, they stay for the lifetime of the ruleengine OHX process.
  Those variables might be used to carry information across single rule exections.
* **Allow network access**: The `os.getURL` method will only work if this option is enabled.
* **Limit run time**: Set the maximum run time in seconds. Default is 5 seconds.

## Acknowledgement

QuikJS is developed on https://bellard.org/quickjs/ by Fabrice Bellard and Charlie Gordon.

Maintainer: David Gräff, 2019

