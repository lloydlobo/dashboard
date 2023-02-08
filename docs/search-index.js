var searchIndex = JSON.parse('{\
"dashboard":{"doc":"dashboard","t":[3,4,13,17,13,13,13,13,6,13,13,13,13,11,11,11,11,11,11,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,5,11,11,11,11,11,11,11,11,11,5,11,11],"n":["App","AppError","CrossbeamError","DESC_WC","Io","LogicBug","Parsing","RegexError","Result","SerdeError","UnwrapError","XshellError","XshellIo","borrow","borrow","borrow_mut","borrow_mut","clone","clone_into","config","db","deref","deref","deref_mut","deref_mut","deserialize","drop","drop","fmt","fmt","fmt","from","from","from","from","from","from","from","from","init","init","into","into","main","provide","serialize","source","to_owned","to_string","try_from","try_from","try_into","try_into","try_main_refactor","type_id","type_id"],"q":["dashboard","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["","<code>AppError</code> Instead of cloning the <code>std::io::Error</code>, we can …","An error occurred while performing an I/O operation across …","Word count limit for description.","An error occurred while performing an I/O operation","An error occurred in the code logic","An error occurred while parsing input","An error occurred with a regular expression","<code>Result&lt;T, E&gt;</code>","An error occurred while serializing or deserializing with …","Catch the panic and return a value of","An error occurred while interacting with the <code>xshell</code> …","An error occurred while performing an I/O operation with …","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","","","Returns the argument unchanged.","","","","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","<code>main</code> entrypoint.","","","","","","","","","","","",""],"i":[0,0,6,0,6,6,6,6,0,6,6,6,6,1,6,1,6,1,1,1,1,1,6,1,6,1,1,6,1,6,6,1,6,6,6,6,6,6,6,1,6,1,6,0,6,1,6,1,6,1,6,1,6,0,1,6],"f":[0,0,0,0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],[1,1],[[]],0,0,[2],[2],[2],[2],[[],[[3,[1]]]],[2],[2],[[1,4],[[3,[5]]]],[[6,4],[[3,[5]]]],[[6,4],[[3,[5]]]],[[]],[7,6],[8,6],[[]],[[[10,[9]]],6],[11,6],[12,6],[13,6],[[],2],[[],2],[[]],[[]],[[],[[14,[6]]]],[15],[1,3],[6,[[17,[16]]]],[[]],[[],18],[[],3],[[],3],[[],3],[[],3],[[],[[3,[6]]]],[[],19],[[],19]],"p":[[3,"App"],[15,"usize"],[4,"Result"],[3,"Formatter"],[3,"Error"],[4,"AppError"],[3,"Error"],[3,"Error"],[3,"Error"],[3,"Arc"],[3,"Error"],[4,"Error"],[4,"ParserError"],[6,"Result"],[3,"Demand"],[8,"Error"],[4,"Option"],[3,"String"],[3,"TypeId"]]},\
"parser":{"doc":"parser","t":[13,13,13,13,13,4,13,13,13,13,13,13,4,4,13,13,13,6,13,13,11,11,11,11,11,11,11,11,14,14,11,11,11,11,11,11,11,11,11,0,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,3,13,4,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,5,11,11,11,11,11,5,5,11,11,11,11,11,11,11,11,11],"n":["AnyhowError","Blue","BufferError","ChannelError","Cyan","ErrorColor","Green","InvalidColor","Io","Io","LogicBug","Magenta","ParserError","PrinterError","PrinterError","Red","RegexError","Result","TermcolorError","Yellow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","clone","clone_into","comment_block","comment_block_dyn","deref","deref","deref","deref_mut","deref_mut","deref_mut","drop","drop","drop","findrepl","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","from","from","from","from","init","init","init","into","into","into","is_channel_error","provide","provide","source","source","to_owned","to_string","to_string","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","CommentBlock","End","Marker","Start","borrow","borrow","borrow_mut","borrow_mut","clone","clone","clone_into","clone_into","default","default","deref","deref","deref_mut","deref_mut","drop","drop","eq","eq","fmt","fmt","fmt","from","from","get_block_positions","init","init","into","into","new","replace","replace_par","to_owned","to_owned","to_string","try_from","try_from","try_into","try_into","type_id","type_id"],"q":["parser","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","parser::findrepl","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["An error occurred using the anyhow library","","An error occurred while using the termcolor buffer","An error occurred using the crossbeam library","","The ErrorColor enum represents the different colors that …","","An error occurred with an invalid color","An error occurred while performing an I/O operation","An error occurred while performing an I/O operation","An error occurred in the code logic","","The <code>ParserError</code> enum represents the different errors that …","<code>PrinterError</code> enum represents the different errors that can …","An error occurred in the printer","","An error occurred in the regex engine","<code>Result&lt;T, E&gt;</code> is an alias for <code>anyhow::Result</code> with …","An error occurred while using the termcolor library","","","","","","","","","","The macro <code>comment_block</code> generates the start and end marker …","Macro <code>comment_block_dyn</code> macro accepts three arguments: …","","","","","","","","","","findrepl","","","","","","","","Returns the argument unchanged.","","","","","Returns the argument unchanged.","","","Returns the argument unchanged.","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Returns <code>true</code> if the parser error is <code>ChannelError</code>.","","","","","","","","","","","","","","","","","<code>CommentBlock</code> is a struct that holds information about a …","The end of a comment block section.","<code>Marker</code> is an enumeration of marker values, <code>Start</code> and <code>End</code>. …","The start of a comment block section.","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the line positions of start and end markers for …","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Creates a new <code>CommentBlock</code>.","<code>replace</code> function first opens the file and reads its …","This version uses <code>rayon::par_iter</code> to bring the necessary …","","","","","","","","",""],"i":[3,1,6,3,1,0,1,6,3,6,3,1,0,0,3,1,3,0,6,1,3,6,1,3,6,1,1,1,0,0,3,6,1,3,6,1,3,6,1,0,3,3,6,6,1,3,3,3,3,3,3,6,6,6,6,1,3,6,1,3,6,1,3,3,6,3,6,1,3,6,3,6,1,3,6,1,3,6,1,0,20,0,20,20,21,20,21,20,21,20,21,20,21,20,21,20,21,20,21,20,21,20,20,21,20,21,0,20,21,20,21,21,0,0,20,21,20,20,21,20,21,20,21],"f":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[1,1],[[]],0,0,[2],[2],[2],[2],[2],[2],[2],[2],[2],0,[[3,4],5],[[3,4],5],[[6,4],5],[[6,4],5],[[1,4],5],[7,3],[[[9,[8]]],3],[[]],[6,3],[8,3],[10,3],[8,6],[[]],[11,6],[12,6],[[]],[[],2],[[],2],[[],2],[[]],[[]],[[]],[3,13],[14],[14],[3,[[16,[15]]]],[6,[[16,[15]]]],[[]],[[],17],[[],17],[[],18],[[],18],[[],18],[[],18],[[],18],[[],18],[[],19],[[],19],[[],19],0,0,0,0,[[]],[[]],[[]],[[]],[20,20],[21,21],[[]],[[]],[[],20],[[],21],[2],[2],[2],[2],[2],[2],[[20,20],13],[[21,21],13],[[20,4],5],[[20,4],5],[[21,4],5],[[]],[[]],[[22,22,22],23],[[],2],[[],2],[[]],[[]],[17,21],[[22,21,24],23],[[22,21,24],23],[[]],[[]],[[],17],[[],18],[[],18],[[],18],[[],18],[[],19],[[],19]],"p":[[4,"ErrorColor"],[15,"usize"],[4,"ParserError"],[3,"Formatter"],[6,"Result"],[4,"PrinterError"],[4,"Error"],[3,"Error"],[3,"Arc"],[3,"Error"],[3,"ParseColorError"],[3,"ColorChoiceParseError"],[15,"bool"],[3,"Demand"],[8,"Error"],[4,"Option"],[3,"String"],[4,"Result"],[3,"TypeId"],[4,"Marker"],[3,"CommentBlock"],[15,"str"],[6,"Result"],[3,"Path"]]},\
"xtask":{"doc":"Code utilized and modified from matklad/cargo-xtask","t":[6,17,6,5,5,5,5,5,5,5,5,5,5,5],"n":["DynError","PKG_NAME","Result","dir_docs","dist_binary","dist_dir","dist_doc_xtask","dist_manpage","main","print_help","project_root","run","run_dist","run_dist_doc"],"q":["xtask","","","","","","","","","","","","",""],"d":["","","","","","","Equivalent shell script","","","","","","Removes a directory at this path, after removing all its …",""],"i":[0,0,0,0,0,0,0,0,0,0,0,0,0,0],"f":[0,0,0,[[],1],[[],[[4,[[3,[2]]]]]],[[],1],[[],[[4,[[3,[2]]]]]],[[],[[4,[[3,[2]]]]]],[[]],[[]],[[],1],[[],[[4,[[3,[2]]]]]],[[],[[4,[[3,[2]]]]]],[[],[[4,[[3,[2]]]]]]],"p":[[3,"PathBuf"],[8,"Error"],[3,"Box"],[6,"Result"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
